# About
Topspot is a simple project started with the goal of learning familiarizing myself with a few important concepts in Rust, include:
- Working with a database ([MySQL](https://docs.rs/mysql/latest/mysql/))
- Networking ([std/net](https://doc.rust-lang.org/std/net/index.html))
- Data serialization and deserialization ([Serde](https://serde.rs/))
- Basic Encryption (password hashing)

# The Data Set
The data set used contains the most streamed spotify songs for 2023 and can be found [here](https://www.kaggle.com/datasets/nelgiriyewithana/top-spotify-songs-2023) on Kaggle.
The data includes some fields that are useful for loosely identify how much a user might enjoy a song (emphasis on loosely). These fields are
- danceability
- valence
- energy
- acousticness
- instrumentalness
- liveness
- speechiness

# Goals
The final version of this project will include the follow functionality:
- The ability to generate song reccomendations based on the results of a questionnaire related to music taste
- Users will be able to create an account and save the results of their most recent questionnaire answers
- Users with an accout will be able to generate random recommendations based on their stored results

# To-DO
- [x] Set up project 
- [x] Add setup option when starting the rust application (will create and populate the MySQL database for you)
- [ ] Networking
- [ ] User accounts
- [ ] Front end
