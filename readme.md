<br/>
<p align="center">
  <h3 align="center">Feedtrac</h3>

  <p align="center">
    A tool designed to elevate communication between students and lecturers.
  </p>
</p>

## Overview
Feedtrac is a feedback tracking system for educational environments that enables students to submit feedback and staff to respond effectively.

## Tech Stack
- **Frontend**: React with TypeScript
- **Backend**: Node.js with Express.js
- **Database**: PostgreSQL with Prisma ORM
- **Containerization**: Docker and Docker Compose
- **Development**: Windows with WSL

## Prerequisites

### Docker Desktop ([Download](https://docs.docker.com/desktop/setup/install))
> [!IMPORTANT]
> Docker requires significant RAM on Windows (16GB+ recommended) as it runs a Linux distribution underneath.
> Consider the frontend-only option if you have limited resources.
> This is generally not an issue on Unix-based systems such as MacOS.

### GitHub Desktop ([Download](https://desktop.github.com/))
> [!NOTE]
> While git command line can be used instead, GitHub Desktop is recommended for easier management of private repositories like this one.

## Setup Instructions

### Clone Repository
1. Open GitHub Desktop
2. Click "File" > "Clone repository"
3. Select repository or enter URL
4. Choose local destination
5. Click "Clone"

### Run Application

#### Option 1: With Docker (Recommended)
```bash
docker-compose up
```
This will start both frontend and backend services together.

#### Option 2: Frontend Only (For UI Work)
If you're only working on the frontend and don't need backend services:

```bash
# install PNPM if you haven't already (Performant node package manager)
npm i -g pnpm

# run the frontend
cd frontend
pnpm install
pnpm start
```

### Accessing the Application
Regardless of setup option, access the application at: http://localhost:3000

## GitHub Desktop Workflow

### Creating a Branch
1. In GitHub Desktop, ensure main is current and updated
2. Click "Current Branch" > "New Branch" 
3. Name it (e.g., "feature/login-page") > "Create Branch"

### Committing Changes
1. After making code changes, check changed files in GitHub Desktop
2. Select files to include in commit
3. Enter commit summary > "Commit to [branch-name]"

### Pushing to GitHub
1. Click "Push origin" to upload commits
2. For new branches, GitHub Desktop will create it remotely

### Creating Pull Requests
1. After pushing, click "Create Pull Request"
2. Complete PR form in browser > "Create pull request"

### Updating Your Branch
1. Switch to main > "Fetch origin" > "Pull origin" 
2. Return to your branch > "Choose a branch to merge into [your-branch]"
3. Select "main" > "Merge main into [your-branch]"

### Resolving Conflicts
1. Open conflicted files from GitHub Desktop in VS Code
2. Edit files to resolve conflicts (look for <<<<<<, =======, >>>>>>>)
3. Save files > mark as resolved > commit the merge

## Troubleshooting
- **Port conflicts**: Check if other services are using ports 3000, 4000, or 5432.
- **Docker issues**: If Docker is using too much RAM, try closing unused containers or use the manual setup.
- **Package issues**: Delete `node_modules` and run `pnpm install` again.

## References
- [React Documentation](https://reactjs.org/docs/getting-started.html)
- [GraphQL Documentation](https://graphql.org/learn/)
- [TypeScript Documentation](https://www.typescriptlang.org/docs/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [Prisma Documentation](https://www.prisma.io/docs/)
- [Apollo Client Documentation](https://www.apollographql.com/docs/react/)
- [Docker Documentation](https://docs.docker.com/)
- [WSL Documentation](https://docs.microsoft.com/en-us/windows/wsl/)