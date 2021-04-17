<!--
 Copyright (C) 2021 Avery
 
 This file is part of rmq-discord-transport.
 
 rmq-discord-transport is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 rmq-discord-transport is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with rmq-discord-transport.  If not, see <http://www.gnu.org/licenses/>.
-->

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

# RMQ Discord Transport
`rmq-discord-transport` is a simple consumer for RabbitMQ that relays information to Discord, in a situation where the result is unimportant, or an isolated worker is instead wanted.

## Configuration
The script takes 2 environment variables into account:
- `RABBITMQ_URI`: required, defines the rabbitmq connection string (for example, `amqp://guest:guest@127.0.0.1`).
- `DEFAULT_WEBHOOK_URI`: optional, is only needed to supply a default webhook link to use when one is not provided.

## Data format
Definitions can be found in [`src/interface.rs`](src/interface.rs). The data is taken as JSON and is expected as follows:

```jsonc
{
    "webhook_uri": "<webhook uri>", // optional
    "payload": {
        "content": "<message content>",
        "username": "<username>", // optional
        "avatar_url": "<avatar_url>", // optional
        "tts": true, // optional
        "embeds": {
            // See https://birdie0.github.io/discord-webhooks-guide/structure/embeds.html
            // These are not internally parsed, so there is no verification of their validity.
            // Any incorrect embeds will simply not be handled.
        }
    },
    "files": [
        {
            "filename": "<file name>",
            "is_spoiler": true, // optional
            "data": [
                /* Data as an array of u8 integers */
            ]
        },
    ]
}
```

## Issues
Create an issue on the [issues tab](https://github.com/starsflower/rmq-discord-transport/issues). Any support/feedback is appreciated.