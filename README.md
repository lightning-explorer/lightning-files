Things to add:

## Organization:
* Refactor directory structure to make things more modular and organized

## Display:
* The thing that displays the current directory that you are in truncates the file name. When you click on it, (it is selected,) it should show the file's entire name so that way if it is copied, you get the whole thing rather than file/.../directory 
* Optional dashboard to see how many files are indexed and can be searched

## Operations:
* Button to create new file/folder
* Smart indexing/search functionality
* Moving a file into a folder


## Context menu:
Should be able to:
* Rename file
* Copy/Paste file
* Delete file


## Sort ability:
* Name
* Date created
* Date modified
* Size
* File type

# Refactor:
* decouple the big mod file and have individual components manage their own mod file
* use Diesel ORM instead of SQLX
* better organize model files
