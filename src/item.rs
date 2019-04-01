#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub title: String,
    pub arg: String,
}

#[derive(Debug)]
pub struct ItemBuilder<TitleType, ArgType> {
    pub title: TitleType,
    pub arg: ArgType,
}

impl ItemBuilder<(), ()> {
    pub fn new() -> Self {
        ItemBuilder { title: (), arg: () }
    }
}

impl ItemBuilder<String, String> {
    pub fn build(self) -> Item {
        Item {
            title: self.title,
            arg: self.arg,
        }
    }
}

impl<TitleType, ArgType> ItemBuilder<TitleType, ArgType> {
    pub fn title<S: Into<String>>(self, title: S) -> ItemBuilder<String, ArgType> {
        ItemBuilder {
            title: title.into(),
            arg: self.arg,
        }
    }
    pub fn arg<S: Into<String>>(self, arg: S) -> ItemBuilder<TitleType, String> {
        ItemBuilder {
            title: self.title,
            arg: arg.into(),
        }
    }
}
