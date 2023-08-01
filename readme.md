# Welcome to the warhammer roster app

The purpose of this app is to help people build an army for playing warhammer.

If you have any suggestions on changes or improvements feel free to raise an issue.

# How to use it?

Once I have the app to a certain level I will begin to create releases and add them to the git repo, but if there are not any in place yet, or you want to use the latest changes you can build it yourself.

This app is built with rust, so you will need to setup your machine to run rust, you can find a guide on how to do that [here](https://www.rust-lang.org/tools/install#:~:text=To%20start%20using%20Rust%2C%20download,see%20%22Other%20Installation%20Methods%22.), its not that complicated compaired to other languages.

You will also need to create a .env file which has the connection details for your mongodb repository, it should look like this:

`MONGODB_URI="mongodb://user:password@127.0.0.1:27017/"`
replace user and password with your actual username and password, and change the ip and port if you are not connecting to a local mongodb.

once you have done that you can execute:

`cargo run`

and it will import the contents of the json files in the `data` directory into your mongodb. It will only import if the unit is not already present, so if a new version is released with updated details you may want to drop your database and let it rebuild to get the latest changes.
