# 3. Use SQLite as persistent storage

Date: 2023-12-29

## Status

Accepted

## Context

We need to store the data for a user in a persistent way. We want the user to be
able to store their tasks locally. We also want the setup to be simple and not
require a lot of configuration.

## Decision

We will use SQLite as the persistent storage for the application.

## Consequences

SQLite is a simple database, and it's data is stored as a file. This means that
a user just has to access this file to get their data. The setup is simple,
since it's just file creation. They can also have different databases for
different needs (such as work and personal) or different users. The file that is
used can then be switched with an environment variable or a program argument. In
addition, there isn't a need for multiple users to access the database at the
same time since `twili` is currently a command-line application, so that isn't
an issue.

Because of the way the architecture is designed, it shouldn't be hard to switch
to another database when the need arises. SQLite should fulfill our needs for
the time being, though.