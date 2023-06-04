// The context (ctx) object to be passed in via middleware on requests.
// Useful to have things like the unique user_id here, and anything else that might
// need to be passed in on all the pages. Different from the User object as that
// might need things like the password

#[derive(Clone, Debug)]
pub struct Ctx {
    username: String,
}

// Constructor.
impl Ctx {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

// Property Accessors.
impl Ctx {
    pub fn username(&self) -> String {
        self.username.clone()
    }
}
