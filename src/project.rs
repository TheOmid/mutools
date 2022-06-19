use crate::*;

// Core project type
// Contains metadata like the project data file's location, and manages loading/saving of project data.
pub struct Project {
    project_data: ProjectData,
    data_file_path: std::string::String,
}

// Raw serializable project type representing the set of sounds in a project, and any project metadata.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectData {
    sound_buffers: std::collections::HashSet<i32>,
}

impl ProjectData {
    pub fn load_from_file(file: std::fs::File) -> Result<ProjectData, errors::DataError> {
        let bson_document_res = bson::Document::from_reader(file);
        match bson_document_res {
            Ok(bson_data) => {
                match bson::from_bson::<ProjectData>(bson::Bson::Document(bson_data)) {
                    Err(_) => Err(errors::DataError::new()),
                    Ok(data) => Ok(data),
                }
            }
            _ => Err(errors::DataError::new()),
        }
    }

    pub fn save_to_file(self, file: std::fs::File) -> Result<(), errors::DataError> {
        let bson_res = bson::to_bson(&self);
        match bson_res {
            Ok(bson::Bson::Document(bson_data)) => match bson_data.to_writer(file) {
                Ok(_) => Ok(()),
                _ => Err(errors::DataError::new()),
            },
            _ => Err(errors::DataError::new()),
        }
    }

    // Hash all data in this object.
    //  -> Allows testing objects for equality.
    pub fn get_data_hash(&self) -> i32 {
        0
    }
}

impl Project {
    pub fn get_project_data(&self) -> &ProjectData {
        &self.project_data
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_and_save_project() -> () {
        ()
    }

}
