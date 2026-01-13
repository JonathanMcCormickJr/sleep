# Sleep

A minimal Rust-based RESTful API for StackOverflow-style Q&A sites. 

Developed by Jonathan McCormick, Jr and LGR (as applicable) within the structure of the `Let's Get Rusty --> Rust Developer Bootcamp --> API Project --> Axum API`. For the assignment details and LGR contributions, please refer to the [bootcamp API project folder](https://github.com/letsgetrusty/bootcamp/tree/master/4.%20Projects/2.%20API-axum). 

## Features
1. `Question` creation, retrieval & deletion
2. `Answer` creation, retrieval & deletion


## TODOs
- [ ] Complete the project MVP as assigned in the bootcamp, including all required tests.
    - [x] Stage 1
        - [x] Step 1
        - [x] Step 2
    - [x] Stage 2
        - [x] Step 1
        - [x] Step 2
    - [ ] Stage 3
        - [x] Step 1
        - [ ] Step 2
        - [ ] Step 3
        - [ ] Check for unresolved TODOs in codebase.
        - [ ] Build and test MVP
- [ ] Audit and updates all dependencies to their latest stable versions.
- [ ] Change all UUIDs to type `uuid::Uuid` instead of `String`.
- [ ] Refactor code to improve modularity and readability.
- [ ] Implement proper error handling throughout the application.
- [ ] Add logging for better traceability and debugging.
- [ ] Set up database migrations for easier schema management.
- [ ] Add input validation for all endpoints.
- [ ] Implement rate limiting to prevent abuse.
- [ ] Bring all HTTP response codes into alignment with RESTful API best practices: https://www.rfc-editor.org/rfc/rfc9110#name-delete. e.g. `DELETE /answer` should return something in line with the following RFC excerpt:
    > If a DELETE method is successfully applied, the origin server SHOULD send
    >    - a 202 (Accepted) status code if the action will likely succeed but has not yet been enacted,
    >    - a 204 (No Content) status code if the action has been enacted and no further information is to be supplied, or
    >    - a 200 (OK) status code if the action has been enacted and the response message includes a representation describing the status.
- [ ] Add pagination to GET endpoints.
- [ ] Add authentication and authorization.
- [ ] Add a feature for users to edit their own questions and answers.
- [ ] Add a feature for users to upvote and downvote questions and answers.
- [ ] Create a way to request sorted results (e.g., by date created, by popularity).
- [ ] Make this a full website with a frontend.
- [ ] Acheive >90% test coverage across the entire codebase.
- [ ] Ensure the project meets the guidelines in [`std_new`](https://github.com/JonathanMcCormickJr/std_new).
- [ ] Deploy the application to a cloud service with TLS via Certbot.
- [ ] Pentest the deployed application for security vulnerabilities.
- [ ] Optimize performance for high traffic scenarios.
- [ ] Write comprehensive documentation for the codebase and API endpoints.
- [ ] Set up continuous integration and continuous deployment (CI/CD) pipelines.
- [ ] Monitor application health and set up alerting for critical issues.
- [ ] Make a post in the LGR Discord channel summarizing the project and linking to the GitHub repository.
- [ ] Share the project on social media and relevant forums to gather feedback and showcase skills.
