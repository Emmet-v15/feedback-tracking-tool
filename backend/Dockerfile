FROM node:20-slim AS base

# Install OpenSSL for Prisma and curl for pnpm installation
RUN apt-get update -y && apt-get install -y openssl curl

# Install pnpm using npm (simpler approach)
RUN npm install -g pnpm@latest

WORKDIR /app

COPY package.json ./
COPY pnpm-lock.yaml ./
COPY prisma ./prisma/

# Copy the the application
COPY . .

# Install dependencies with pnpm
RUN pnpm install

# Create empty .env file if it doesn't exist
RUN touch .env

# Build the application (tsconfig.json should generate .js files to dist/)
RUN pnpm run build

EXPOSE 4000

# Use the compiled JavaScript in production
CMD ["pnpm", "start"]