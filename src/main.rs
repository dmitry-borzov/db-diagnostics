// ============== SETTINGS ===============================
const INPUT: &str = r#"
DOCUMENT_ID               NOT NULL NUMBER         
DOCUMENT_PATH             NOT NULL VARCHAR2(2000) 
DOCUMENT_STORAGE_TYPE_ID           NUMBER         
DOCUMENT_STORAGE_TYPE              VARCHAR2(100)  
DOCUMENT_SOURCE_ID                 NUMBER         
DOCUMENT_SOURCE                    VARCHAR2(100)  
DOCUMENT_TYPE_ID                   NUMBER         
DOCUMENT_TYPE_DESCRIPTION          VARCHAR2(100)  
DISPUTE_RESOLUTION_ID     NOT NULL NUMBER         
DISPUTE_ID                         CLOB         
SEQUENCE_NUMBER           NOT NULL NUMBER         
HIDDEN                    NOT NULL VARCHAR2(1)    
CREATE_DATE               NOT NULL DATE           
CREATE_BY_UID             NOT NULL VARCHAR2(50)   
LAST_UPDATE_DATE                   DATE           
LAST_UPDATE_BY_UID                 VARCHAR2(50)
    "#;
const DB_OBJECT_NAME: &str = "V_DISPUTE_RES_ATTACHMENTS";
const SCHEME_NAME: &str = "CARRIER";
const IS_VIEW: bool = true;
// ============== End of SETTINGS =========================

fn main() {
    let mut fields = "".to_string();
    let input_lines = INPUT.split("\n").filter(|x| !x.is_empty());
    for line in input_lines {
        let splitted_line = line
            .split(' ')
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();

        if splitted_line.len() == 0 { continue; }

        let field_name = splitted_line[0];
        let mut field_type = splitted_line[splitted_line.len() - 1].to_string();
        let offset = field_type.find("(").unwrap_or(field_type.len());
        field_type.truncate(offset);
        let nullable = if splitted_line.contains(&"NOT") {
            "false"
        } else {
            "true"
        };
        let result_line = format!(
            r#"                        new DatabaseObjectArgument {{ Name = "{}", Type = "{}", Nullable = {}, }},"#,
            field_name, field_type, nullable
        );
        fields.push_str(&result_line);
        fields.push_str("\n");
    }

    fields = fields.trim_end_matches("\n").to_string();

    let object_type = if IS_VIEW { "View" } else { "Table" };
    println!(
        r#"
public static DatabaseObject {}
{{
    get
    {{
        return new DatabaseObject
        {{
            ObjectType = DatabaseObjectType.{},
            Owner = "{}",
            Name = "{}",
            Arguments = new List<DatabaseObjectArgument>
                    {{
{}
                    }}
        }};
    }}
}}
"#,
        DB_OBJECT_NAME, object_type, SCHEME_NAME, DB_OBJECT_NAME, fields
    );
}
