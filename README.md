# Pleco Flashcards Tool
Streamlined way to review chinese characters.

## Features
* Preview the traditional characterset *after* you study a card
* Randomized ways to review a word
    * Type in the pinyin
    * Flash card
* Score based system to prioritize order

## Usage
Please use the `.txt` export and NOT the `.xml` export.
Choose charset `utf8`, choose both character sets, choose **card definitions**, **dictionary definitions** and **(remap if unexportable)**

## Score-Based System
The score-based system is as follows:
* Each card starts with **0** as the score.
* If you get it right,
    * The score increases by **1** (for now).
    * **Later**: The score increases by **1** or **2** based on your strength of knowledge (remembered, barely remembered)
* If you get it wrong,
    * The score decreases by **1** (for now).
    * **Later**: The score decreases by **1** or **2** based on user input.

After the end of each test, the score gets **normalized**.
Every score will be subtracted by the lowest scoring card.
For example, if the lowest card had a score of `-1`, every card will be subtracted `-1` (in other words +1).