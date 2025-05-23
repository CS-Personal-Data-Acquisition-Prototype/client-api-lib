# client-api-lib
A Rust library to facilitate sending and receiving requests between the web client frontend and online backend.

### Local Testing
- Navigate into the tcp-client folder
  - `cd ./tcp-client`

- Download all library targets
  - `cargo build`

- Run the application
  - `cargo run`

Note: This repository is meant to be used as an external crate for the UI layer. Running the application as is will not send any requests.


### External Crate Usage
For use as an external crate, first specify the IP address and port number where all requests should be sent to in the `.env` file located under `client-api-lib/tcp-client`. Example usage with localhost and Port 80:
```
API_BASE_URL = http://127.0.0.1:80
```

Next, build the application and add the library as a dependency in the relevant Cargo.toml file.
```
[dependencies]
tcp-client = { path = "path/to/client-api-lib/tcp-client" }
```

In all relevant files, include the API as an external crate and import the needed endpoints.
```rust
extern crate client;
use client::api::{auth, sensor, sensor_session_data, session_sensor, session, user};
```

All requests to the server require an HTTP Client, which can be retrieved by calling `get_client()` and used for all subsequent requests.
```rust
let client = client::get_client();
let (status, body) = user::create_user(&client, &username, &password).await;
```


# License Notice
To apply the Apache License to your work, attach the following boilerplate notice. The text should be enclosed in the appropriate comment syntax for the file format. We also recommend that a file or class name and description of purpose be included on the same "printed page" as the copyright notice for easier identification within third-party archives.

    Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
    Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
    
    http://www.apache.org/licenses/LICENSE-2.0
    Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.