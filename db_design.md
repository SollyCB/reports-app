# A Preliminary Plan For The MySQL Database Structure

*From a gentleman on stack overflow*

> You should ask youself whether those seperate groups are going to need the same table structure. In a relational database, seperate tables should represent seperate entities. Having your database structured this way keeps entities with like fields structured together and creates a logical flow in a database. 

> As much as possible, try to structure your database so that seperate entities have their own tables. Even if this were to somehow cost some neglegable extra performance, it would save you hours upon hours when writing queries or trying to expand the database.

> **Trust me, as someone who has worked with databases where tables contain information that logically should exist as five or six tables, you will be saving yourself a lot of headache...**

## Pupils

*These tables will persist by year, but will have a field which indicates whether they graduated or left*

Of course there has a to a be a table which contains all pupils. Rows should be obvious as to what they should be: 

- Name
- DOB
- Medical Stuff
- Class
- Subjects (for fetching individual reports)

|       |   id  |   DOB |   Guardian    |
|-------|-------|-------|-------|
| John Smith | 0 | 27/10/2001 | Ron Smith |
| Jo Friend | 1 | 25/9/2003 | Gay Friend |

## Classes

*These tables will be created yearly, named by the year that they were created*

There will be a table which contains a list of classes, with a column which contains a list of the names and ids of the pupils in the class.

|       | pupils |
|-------|-------|
| 7K | Ron, John|
| 8K | Tom, Dom |

Each class will have its own table which will contain the timetable for the class, (special cases will have to be made for set classes, such as maths and english...). These tables will have columns for each time and rows for each day.
    
*Table Name: 7K_2022_2023*

|   | 0900 | 1000 | 1100 |
|---|---|---|---|
| Monday| Bio | Maths | English |
| Tues| Some | Dumb | Lesson |

## Subjects 

*These tables will be created yearly, named by the year that they were created*

There will be a table for each subject, which will have rows for Autumn, Winter, Spring and Summer terms. The columns will contain the names of the pupils that take these subjects. The fields will contain the pupils' reports. 

*Table Name: bio_2022_2023*

|   | Autumn | Winter | Spring | Summer |
|---|---|---|---|---|
|Jon|Ooo! Very well done|Tsk Tsk, slipping|Uh Oh, What's that?|Mmmmmm...|
|Tom|Just awful|Oh My! Dreadful!|My Arse! Look At IT!| Yikers it's pink....|

