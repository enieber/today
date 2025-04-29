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

## View using Marmite

- to view list tasks use [marmite project](https://github.com/rochacbruno/marmite/) like: `marmite $TODAY_BASE_FILE live-tasks --serve`

```bash
  # alias to use with marmite live
  alias today-show="marmite $TODAY_BASE_FILE --watch --serve /home/$USER/live-task"
```

![image](https://github.com/user-attachments/assets/dea2fb3c-07ad-4fcd-b391-1686bad92d52)
