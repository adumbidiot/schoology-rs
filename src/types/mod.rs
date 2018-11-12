#[derive(Deserialize, Debug)]
pub struct Group{
	pub id: String,
	pub title: String,
	pub description: String,
	website: String,
	access_code: Option<String>,
	category: String,
	group_code: String,
	privacy_level: String,
	picture_url: String,
	school_id: String,
	building_id: String,
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
	name_title_show: u32,
	pub name_first: String,
	name_first_preferred: String,
	use_preferred_first_name: String,
	pub name_middle: Option<String>,
	name_middle_show: u32,
	pub name_last: String,
	picture_url: String,
	pub gender: Option<String>,
	pub position: Option<String>,
	child_uids: Option<String>,
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
	user_like_action: bool,
	realm: String,
	group_id: u32,
	pub num_comments: u32,
}

#[derive(Deserialize, Debug)]
pub struct UpdateList{
	pub update: Vec<Update>
}