use iced::{
    widget::{button, column, row, text_input},
    Length,
};
use iced_kim::{table, table_head, table_index, TableHead, TableRow};

fn main() -> iced::Result {
    iced::run("A table example", Example::update, Example::view)
}

struct Example {
    table_datas: Vec<Person>,
    table_heads: Vec<TableHead>,
    row_selected: Option<i32>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            table_datas: vec![
                Person::new("kim", 18),
                Person::new("tom", 20),
                Person::new("jerry", 19),
                Person::new("lily", 22),
                Person::new("lucy", 25),
                Person::new("harry", 23),
                Person::new("peter", 21),
                Person::new("bill", 24),
            ],
            table_heads: vec![
                table_index("index").width(80), // add a index column to the table, it will be start from 1
                table_head("name", "user name").width(200), // show the table header [user name] with the source data section [name]
                table_head("age", "user age").width(120), // show the table header [user age] with the source data section [age]
            ],
            row_selected: None,
        }
    }
}

/// the table row data structure
#[derive(TableRow, Default, Clone)]
struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: impl Into<String>, age: u32) -> Person {
        Person {
            name: name.into(),
            age,
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    RowSelected(i32),
    ColWidthChanged(String, f32),
    RowMoved { src: i32, dst: i32 },
    ColMoved { src: i32, dst: i32 },

    Add,
    Delete(i32),
    NameChanged(String),
    AgeChanged(String),
}

impl Example {
    pub fn view(&self) -> iced::Element<Message> {
        // * create a new table
        let tb = table(&self.table_heads, &self.table_datas)
            .width(400)
            .height(300)
            .row_height(40)
            // * you can selected row
            .on_row_selected(|row| Message::RowSelected(row).into())
            // * you can change the column width when press and move after hovering over the spliter
            .on_col_width_changed(|name, width| Message::ColWidthChanged(name.into(), width).into())
            // * you can move row when press and drag the row
            .on_row_moved(|a| {
                Message::RowMoved {
                    src: a.src_row_index,
                    dst: a.dst_row_index,
                }
                .into()
            })
            // * you can move the column when press and drag the column
            .on_col_moved(|a| {
                Message::ColMoved {
                    src: a.src_col_index,
                    dst: a.dst_col_index,
                }
                .into()
            });

        // * get the current selected row value
        let (name_value, age_value) = if let Some(row) = self.row_selected {
            (
                Some(&self.table_datas[row as usize].name),
                Some(self.table_datas[row as usize].age),
            )
        } else {
            (None, None)
        };

        // * create editers for the selected row
        // * create a new text input for name
        let name = text_input("Please enter name", name_value.map_or("", |v| v))
            .on_input_maybe(if self.row_selected.is_some() {
                Some(|x| Message::NameChanged(x))
            } else {
                None
            })
            .width(Length::Fill);
        // * create a new text input for age
        let age = text_input("Please enter age", &age_value.unwrap_or(0).to_string())
            .on_input_maybe(if self.row_selected.is_some() {
                Some(|x| Message::AgeChanged(x))
            } else {
                None
            })
            .width(Length::Fill);
        // * create a new add button
        let add = button("Add").on_press(Message::Add).width(Length::Fill);
        // * create a new delete button
        let del = button("Delete")
            .on_press_maybe(if let Some(row) = self.row_selected {
                Some(Message::Delete(row))
            } else {
                None
            })
            .width(Length::Fill);

        // * layout inputs and buttons
        let editers = column![name, age, add, del].spacing(10).width(160);

        // * layout editers and table
        row![editers, tb].spacing(20).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RowSelected(row) => {
                self.row_selected = Some(row);
            }
            Message::RowMoved { src, dst } => {
                let moved = self.table_datas.remove(src as usize);
                self.table_datas.insert(dst as usize, moved);
            }
            Message::ColMoved { src, dst } => {
                let moved = self.table_heads.remove(src as usize);
                self.table_heads.insert(dst as usize, moved);
            }
            Message::ColWidthChanged(name, width) => {
                let head = self.table_heads.iter_mut().find(|x| x.name.eq(&name));
                if let Some(head) = head {
                    head.width = (width).into();
                }
            }
            Message::Add => {
                self.table_datas.push(Person::default());
            }
            Message::Delete(index) => {
                let index = index as usize;
                self.table_datas.remove(index);
                if index >= self.table_datas.len() {
                    self.row_selected = None;
                }
            }
            Message::NameChanged(name) => {
                if let Some(row) = self.row_selected {
                    self.table_datas[row as usize].name = name;
                }
            }
            Message::AgeChanged(age) => {
                if let Some(row) = self.row_selected {
                    if age.len() == 0 {
                        self.table_datas[row as usize].age = u32::MIN;
                    } else if let Ok(age) = age.parse() {
                        self.table_datas[row as usize].age = age;
                    }
                }
            }
        }
    }
}
