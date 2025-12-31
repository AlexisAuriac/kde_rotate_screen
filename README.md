# kde_screen_rotate

Simple tool to rotate the screen, only works on kde.

Ergonomic wrapper around `kscreen-doctor`.

# install

```bash
cargo install --path .
```

# usage

## list outputs

```bash
$ kde_rotate_screen outputs
eDP-1	enabled	2560x1600@300	normal
```

## reset

```bash
kde_rotate_screen reset
# or
kde_rotate_screen reset --output eDP-1
```

## rotate

clockwise
```bash
kde_rotate_screen rotate
# or
kde_rotate_screen rotate clockwise
# or
kde_rotate_screen rotate clockwise --output eDP-1
```

counter clockwise
```bash
kde_rotate_screen rotate counter-clockwise
```

## set screen orientation

normal
```bash
kde_rotate_screen orient
# or
kde_rotate_screen orient normal
# or
kde_rotate_screen orient normal --output eDP-1
```

left
```bash
kde_rotate_screen orient left
```

right
```bash
kde_rotate_screen orient right
```

inverted
```bash
kde_rotate_screen orient inverted
```
