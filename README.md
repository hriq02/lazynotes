# lazynotes

just a simple tool to organize for your files, can be used to open projects more eazly <br />
this is a simple project, you can modfy as you want or use it as a base for something else <br />

# how to use it?
you add the path of the exe folder to your path variable, and just type lzn {name of the file with extension, you dont need it}<br />
at the start if you type the name and its not listed it will ask if you want to create it in the default folder witch at first will be in the exe folder called "notes"<br />
but you can change the default folder by typing -dp {path}<br />
also if you just want the path of the folder you can just type -gp <br />
you can print the content using the sign -p <br />
you can set the default editor that will open in any file by typing -de {name of the editor}<br />
but you can open with a specific editor with -e {editor} <br />
the default path is where you notes go if you dont add them to the list <br />
# side note
this project is not to take seriously, is just for fun and for me to learn new rust and new stuff <br />
is still on development, so any suggestion will be welcome<br />

# here are the following commands:
| short command |     |   long command   |         description           |
|     ---       | --- |      ---         |           ---                 |
|     -e        | or  | --editor         |  opens with specific editor   |
|     -a        | or  | --add            |  to add path to te list       |
|     -p        | or  | --print          |  prints content from the file |
|     -gp       | or  | --get_path       |  gets the path from the file  |
|     -de       | or  | --default_editor |  sets the default editor      |
|     -dp       | or  | --default_path   |  sets the default path        |
