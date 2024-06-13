## How to run your application
```cargo run``` to start server will listen on port 3001
```cd chat-frontend && npm i && npm start``` to start frontend on port 3000 while server is running

Tests could be executed via ```cargo test``` and with running server ```cargo test -q quick_dev -- --nocapture --include-ignored```

## Overview of your approach
After Ive got information that your test case expect usage of Axum framework, 
Ive found a lesson on it and followed it to learn how-to work with it, how to
handle routes, and errors, and how simple requests are handled. Also how CRUD apis
could be implemented.

After Ive got assigment, my thoghts were, it should be pretty simple just few 
API calls, at least that should be sufficient for that task.

So I've started with data model, and model controller to describe functions 
to work with it - create, list, delete. Update one was a bit tricky as it required 
in place update of message, and in order for it to correctly work later 
with frontend it should also return new message, and error handling for delete 
and update. After that code to render Json responces and map routes to HTTP 
request types. During work I've utilised quick_dev test in order to execute checks
agains freshly changed code (main code and quick dev tests were running with cargo watch)

After I got working API, Ive started to look into simple typescript chat app,
making it to list messages and send messages was pretty easy and straighforward,
but update and delete required additional handling.

## Brief(!) assessment of strengths and weaknesses
It works, it's simple, it's readable  
logging could be better, frontend naming scheme does not match backend one

## What you would do if you had more time
Add login, and cookies handling, more verbose server side logging, tracing,
monitoring, docker image, unit tests for frontend, and same naming convention