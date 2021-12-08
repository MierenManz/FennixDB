use super::super::errors::CreationError;
use super::super::errors::InsertionError;
use super::file_manager::FileManager;
use super::schemes::Row;
use super::schemes::Table;
use std::fs::OpenOptions;
use std::path::Path;

struct Permissions {
    read: bool,
    write: bool,
    create: bool,
}

pub struct Database {
    name: Option<&'static str>,
    file: Option<FileManager>,
    in_memory: bool,
    permissions: Permissions,
    scheme: Vec<Table>,
    // not implemented!
    // indices: Vec<Index>
}

pub struct DatabaseDescriptor {
    pub name: Option<&'static str>,
    pub file: Option<&'static str>,
    pub read: bool,
    pub write: bool,
    pub create: bool,
}

impl Database {
    pub fn new<'a>(database_descriptor: DatabaseDescriptor) -> Result<Self, CreationError> {
        if !database_descriptor.read && !database_descriptor.write {
            return Err(CreationError::new("Not allowed to read or write to file!"));
        }

        let maybe_file_handle = match database_descriptor.file {
            Some(file_name) => {
                let path = Path::new(file_name);
                if !path.is_file() {
                    return Err(CreationError::new("Path is not a file"));
                }

                if !path.exists() && !database_descriptor.create {
                    return Err(CreationError::new(
                        "File does not exist and is not allowed to be created",
                    ));
                }
                let result_file_handle = OpenOptions::new()
                    .read(database_descriptor.read)
                    .append(database_descriptor.write)
                    .create(database_descriptor.create)
                    .open(path);

                let file_handle = match result_file_handle {
                    Ok(handle) => handle,
                    Err(_) => return Err(CreationError::new("Could not open file with settings")),
                };

                Some(file_handle)
            }
            None => None,
        };

        let maybe_file_manager = match maybe_file_handle {
            Some(file_handle) => Some(FileManager::new(file_handle)),
            None => None,
        };
        Ok(Self {
            name: database_descriptor.name,
            file: maybe_file_manager,
            permissions: Permissions {
                read: database_descriptor.read,
                write: database_descriptor.write,
                create: database_descriptor.create,
            },
            in_memory: database_descriptor.file.is_none(),
            scheme: Vec::new(),
        })
    }
}
