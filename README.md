# Today

![today logo](today-logo.svg)

Today is manager using markdown with manager todo task.

## Features

- create new file markdown with date format.
- get weather from CPTEC using brazil-api.
- manager tasks with todo.

## How to use

- clone project
- build release
- use bin file
- config env:

  ```bash
  export TODAY_BASE_FILE=/home/$USER/tasks
  export TODAY_CITY_CODE=244
  ```
- to view list tasks use [marmite project](https://github.com/rochacbruno/marmite/) like: `marmite $TODAY_BASE_FILE live-tasks --serve`

## License



MIT
