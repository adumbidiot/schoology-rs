#[derive(Deserialize, Debug)]
pub struct Group{
	pub id: String,
	pub title: String,
	pub description: String,
	pub website: String,
	pub access_code: Option<String>,
	pub category: String,
	pub group_code: String,
	pub privacy_level: String,
	pub picture_url: String,
	pub school_id: String,
	pub building_id: String,
}

#[derive(Deserialize, Debug)]
pub struct GroupList{
	pub group: Vec<Group>,
}

#[derive(Deserialize, Debug)]
pub struct User{
	pub uid: String, 
	pub id: u32,
	pub name_title: String,
	pub name_title_show: u32,
	pub name_first: String,
	pub name_first_preferred: String,
	pub use_preferred_first_name: String,
	pub name_middle: Option<String>,
	pub name_middle_show: u32,
	pub name_last: String,
	pub picture_url: String,
	pub gender: Option<String>,
	pub position: Option<String>,
	pub child_uids: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UserList{
	pub user: Vec<User>,
}

#[derive(Deserialize, Debug)]
pub struct Update {
	pub id: u32,
	pub body: String,
	pub uid: u32,
	pub created: u32,
	pub likes: u32,
	pub user_like_action: bool,
	pub realm: String,
	pub group_id: u32,
	pub num_comments: u32,
}

#[derive(Deserialize, Debug)]
pub struct UpdateList{
	pub update: Vec<Update>
}