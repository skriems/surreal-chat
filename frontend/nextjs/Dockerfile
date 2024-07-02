FROM node:20-alpine AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

ENV NEXT_TELEMETRY_DISABLED 1

FROM base AS build
COPY . /usr/src/app
WORKDIR /usr/src/app

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

RUN pnpm run -r build

RUN pnpm deploy --filter=nextjs --prod /prod/nextjs

# RUN pnpm deploy --filter=app2 --prod /prod/app2

FROM base AS nextjs
COPY --from=build /prod/nextjs /prod/nextjs
WORKDIR /prod/nextjs
EXPOSE 3000
CMD [ "pnpm", "start" ]

# FROM base AS app2
# COPY --from=build /prod/app2 /prod/app2
# WORKDIR /prod/app2
# EXPOSE 8001
# CMD [ "pnpm", "start" ]
