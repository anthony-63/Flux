# Flux Map (.flm) file spec

struct sized_data {
    u16 size,
    [u8;size] data
}
struct map {
    sized_data artist
    sized_data song_name
    sized_data mapper
    sized_data map_data
    [u8] mp3_data
}