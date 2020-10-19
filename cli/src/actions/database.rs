// Copyright 2019 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::CliError;

use diesel::{connection::Connection as _, pg::PgConnection};

use grid_sdk::grid_db::migrations::run_postgres_migrations;

pub fn run_migrations(database_url: &str) -> Result<(), CliError> {
    let connection = PgConnection::establish(database_url)
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;

    run_postgres_migrations(&connection).map_err(|err| CliError::DatabaseError(err.to_string()))?;

    info!("Successfully applied migrations");

    Ok(())
}
