# README - archive_me_plz

A BLAZINGLY 🚀 FAST 🏍 mass file archiver, written in Rust 🦀

Early stages of development, but the goal is to have a fast, efficient, and easy to use file archiver that can handle large amounts of files and directories.

Built for the unfortunate workplaces where your buisness decided to use Windows network drives or Excel files as a database.
We can make it better - Time to clean up the mess and archive those files!

## Usage

```bash
archive_me_plz <qualifier> <oldest_to_keep> <directory_path>
```

### Example

```bash
archive_me_plz 52 2024-01-01 '/path/to/directory'
```

## Qualifiers

Qualifiers are used to determine if the files within the given directory should be archived or not.
The qualifier is the first argument passed to the program.

This is a work in progress, but the idea is to have a way to filter out whole directories based on a condition.
For example, if you only want to archive directories that have not been accessed in the last 6 months, you could pass in a qualifier that checks the last accessed time of the directory.

---

### WIP

- [ ] Modularize and find places for interface usages where applicable.

- [ ] Add the ability to provide a root starting directory, and to have the qualifer (first argument)
  be the condition on if a sub-directory get's added to the list of "to be searched/actioned"
  directories. Note: this will require testing on which way of crawling directories/dir structures is best.

- [ ] Create printer/logger to make passing in a logger easier.

- [ ] Create a way to pass in a logger.

- [ ] Addition of unit tests and test modules.
