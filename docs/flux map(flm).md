# Flux Map (.flm) file spec

```c
struct sized_data {
    uint16_t size,
    uint8_t data[size]
};
```

```c
struct sized_data_large {
    uint32_t size,
    uint8_t data[size]
};
```

```c
struct map {
    struct sized_data artist
    struct sized_data song_name
    struct sized_data mapper
    struct sized_data_large map_data
    uint8_t mp3_data[];
};
```