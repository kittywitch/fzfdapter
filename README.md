# fzfdapter

The intention of fzfdapter is that it's supposed to be used in a floating terminal to replace something like wofi/rofi/fuzzel/dmenu/...!

```
A PATH and desktop file executor that uses fzf/skim/...

Usage: fzfdapter [OPTIONS]

Options:
  -m, --mode <MODE>...  How to source programs [possible values: all, desktop, path]
  -h, --help            Print help
```

We store the quantity of times a specific application has been opened within $XDG_CACHE_HOME/fzfdapter/cache.msgpack to be able to display most used apps above or below all the others depending upon your configuration.

## Configuration

See ./config.toml.example for an example on how I use it.

## Screenshots

<img width="838" height="199" alt="image" src="https://github.com/user-attachments/assets/3145e94a-60fa-45f4-8d83-e05f5323394c" />

Help/about view

<img width="811" height="1563" alt="image" src="https://github.com/user-attachments/assets/5302b6ab-edec-477d-8c21-65dacac466b3" />

Desktop mode

<img width="861" height="1586" alt="image" src="https://github.com/user-attachments/assets/4999f39c-c9d8-497a-bc42-ce808c4ea32f" />

Path mode

<img width="861" height="1586" alt="image" src="https://github.com/user-attachments/assets/889393fb-d858-45d8-8116-6c351a45373d" />

All mode
