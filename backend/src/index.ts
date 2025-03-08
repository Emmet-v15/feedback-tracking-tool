// Import required packages
import { ApolloServer } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';
import { PrismaClient, UserRole, FeedbackStatus, FeedbackType } from '@prisma/client';

// Initialize Prisma client
const prisma = new PrismaClient();

// Define your GraphQL schema
const typeDefs = `
  enum UserRole {
    ADMIN
    STAFF
    STUDENT
  }

  enum FeedbackStatus {
    OPEN
    IN_PROGRESS
    RESOLVED
    CLOSED
    UNRESOLVED
  }

  enum FeedbackType {
    MODULE
    LECTURE
    PROGRAM
    SCHOOL
    OTHER
  }

  type User {
    id: ID!
    email: String!
    name: String
    role: UserRole!
    createdAt: String!
    updatedAt: String!
    feedback: [Feedback!]
    comments: [Comment!]
  }

  type Feedback {
    id: ID!
    title: String!
    description: String!
    status: FeedbackStatus!
    type: FeedbackType!
    createdAt: String!
    updatedAt: String!
    closedAt: String
    createdBy: User!
    comments: [Comment!]
    tags: [Tag!]
  }

  type Comment {
    id: ID!
    content: String!
    createdAt: String!
    updatedAt: String!
    author: User!
    feedback: Feedback!
  }

  type Tag {
    id: ID!
    name: String!
    feedback: [Feedback!]
  }

  type Query {
    users: [User!]!
    user(id: ID!): User
    feedback(id: ID!): Feedback
    allFeedback: [Feedback!]!
    comments(feedbackId: ID!): [Comment!]!
    tags: [Tag!]!
  }

  type Mutation {
    createUser(email: String!, name: String, role: UserRole): User!
    createFeedback(title: String!, description: String!, type: FeedbackType!, userId: ID!): Feedback!
    updateFeedbackStatus(id: ID!, status: FeedbackStatus!): Feedback!
    addComment(content: String!, userId: ID!, feedbackId: ID!): Comment!
    createTag(name: String!): Tag!
    addTagToFeedback(tagId: ID!, feedbackId: ID!): Feedback!
  }
`;

// Define resolvers that use Prisma to fetch data
const resolvers = {
  Query: {
    users: async () => {
      return prisma.user.findMany();
    },
    user: async (_: any, { id }: { id: string }) => {
      return prisma.user.findUnique({ where: { id } });
    },
    feedback: async (_: any, { id }: { id: string }) => {
      return prisma.feedback.findUnique({ where: { id } });
    },
    allFeedback: async () => {
      return prisma.feedback.findMany();
    },
    comments: async (_: any, { feedbackId }: { feedbackId: string }) => {
      return prisma.comment.findMany({
        where: { feedbackId }
      });
    },
    tags: async () => {
      return prisma.tag.findMany();
    }
  },
  Mutation: {
    createUser: async (_: any, { email, name, role = UserRole.STUDENT }: { email: string, name?: string, role?: UserRole }) => {
      return prisma.user.create({
        data: {
          email,
          name,
          role
        },
      });
    },
    createFeedback: async (_: any, { title, description, type, userId }: { title: string, description: string, type: FeedbackType, userId: string }) => {
      return prisma.feedback.create({
        data: {
          title,
          description,
          type,
          createdBy: {
            connect: { id: userId }
          }
        },
      });
    },
    updateFeedbackStatus: async (_: any, { id, status }: { id: string, status: FeedbackStatus }) => {
      const data: any = { status };
      
      // If we're closing the feedback, set the closedAt date
      if (status === FeedbackStatus.CLOSED || status === FeedbackStatus.RESOLVED) {
        data.closedAt = new Date();
      }
      
      return prisma.feedback.update({
        where: { id },
        data
      });
    },
    addComment: async (_: any, { content, userId, feedbackId }: { content: string, userId: string, feedbackId: string }) => {
      return prisma.comment.create({
        data: {
          content,
          author: {
            connect: { id: userId }
          },
          feedback: {
            connect: { id: feedbackId }
          }
        }
      });
    },
    createTag: async (_: any, { name }: { name: string }) => {
      return prisma.tag.create({
        data: { name }
      });
    },
    addTagToFeedback: async (_: any, { tagId, feedbackId }: { tagId: string, feedbackId: string }) => {
      return prisma.feedback.update({
        where: { id: feedbackId },
        data: {
          tags: {
            connect: { id: tagId }
          }
        },
        include: {
          tags: true
        }
      });
    }
  },
  // Resolve relationships
  User: {
    feedback: async (parent: any) => {
      return prisma.feedback.findMany({
        where: { createdById: parent.id }
      });
    },
    comments: async (parent: any) => {
      return prisma.comment.findMany({
        where: { authorId: parent.id }
      });
    }
  },
  Feedback: {
    createdBy: async (parent: any) => {
      return prisma.user.findUnique({
        where: { id: parent.createdById }
      });
    },
    comments: async (parent: any) => {
      return prisma.comment.findMany({
        where: { feedbackId: parent.id }
      });
    },
    tags: async (parent: any) => {
      const feedback = await prisma.feedback.findUnique({
        where: { id: parent.id },
        include: { tags: true }
      });
      return feedback?.tags || [];
    }
  },
  Comment: {
    author: async (parent: any) => {
      return prisma.user.findUnique({
        where: { id: parent.authorId }
      });
    },
    feedback: async (parent: any) => {
      return prisma.feedback.findUnique({
        where: { id: parent.feedbackId }
      });
    }
  },
  Tag: {
    feedback: async (parent: any) => {
      const tag = await prisma.tag.findUnique({
        where: { id: parent.id },
        include: { feedback: true }
      });
      return tag?.feedback || [];
    }
  }
};

// Create and start the Apollo Server
async function startServer(): Promise<void> {
  const server = new ApolloServer({
    typeDefs,
    resolvers,
  });

  const { url } = await startStandaloneServer(server, {
    listen: { port: 4000 },
    context: async () => ({
      prisma,
    }),
  });

  console.log(`🚀 Server ready at: ${url}`);
}

startServer();