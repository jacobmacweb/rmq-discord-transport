// Copyright (C) 2021 Avery
// 
// This file is part of rmq-discord-transport.
// 
// rmq-discord-transport is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rmq-discord-transport is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with rmq-discord-transport.  If not, see <http://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use std::string::String;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct IncomingTransportData {
    pub webhook_uri: Option<String>,
    pub payload: WebhookData,
    pub files: Option<Vec<File>>
}

#[derive(Serialize, Deserialize)]
pub struct WebhookData {
    pub content: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub tts: Option<bool>,
    pub embeds: Option<Vec<Value>>
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub filename: String,
    pub is_spoiler: Option<bool>,
    pub data: Vec<u8>
}

#[derive(Serialize, Deserialize)]
pub struct WebhookResponse {
    pub id: String
}