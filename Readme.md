## How to run your application
```cargo run``` to start server will listen on port 3001
```cd chat-frontend && npm i && npm start``` to start frontend on port 3000 while server is running

Tests could be executed via ```cargo test``` and with running server ```cargo test -q quick_dev -- --nocapture --include-ignored```

## Overview of your approach
After I received information that your test case expects the usage of the Axum framework, I found a lesson on it and followed it to learn how to work with it, handle routes and errors, and manage simple requests. I also learned how CRUD APIs could be implemented.

When I received the assignment, I thought it should be pretty simple with just a few API calls, which I believed would be sufficient for the task.

So I started with the data model and model controller to describe the functions for working with it: create, list, and delete. The update function was a bit tricky as it required an in-place update of the message. To work correctly with the frontend, it also needed to return the new message and handle errors for both delete and update operations. After that, I wrote code to render JSON responses and map routes to HTTP request types. During the work, I utilized quick_dev test to execute checks against the freshly changed code (the main code and quick_dev tests were running with cargo watch).

After getting the API to work, I started looking into a simple TypeScript chat app. Making it list and send messages was pretty easy and straightforward, but update and delete required additional handling.


## Brief(!) assessment of strengths and weaknesses
It works, it's simple, it's readable  
logging could be better, frontend naming scheme does not match backend one

## What you would do if you had more time
Add login, and cookies handling, more verbose server side logging, tracing,
monitoring, docker image, unit tests for frontend, and same naming convention
