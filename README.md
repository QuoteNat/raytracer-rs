A rust raytracing implementation based on the books/guides found in the [Raytracing in One Weekend Book Series](https://raytracing.github.io/)

Only dependency as far as I'm aware is `gcc`.

Example scenes included below to demonstrate current capabilities:

`cargo run -r scenes/rttnw_final.json` (note: This specific render has a 10th of the samples that are currently coded in for the scene)

![rttnw_final.json](https://cdn.discordapp.com/attachments/1061798205278396416/1068164735905832970/rttnwfinal.png)

`cargo run -r scenes/custom_bubble.json`

![custom_bubble.json](https://i.imgur.com/AT2jUit.png)

`cargo run -r scenes/cornell_fog.json`

![cornell_fog.json](https://cdn.discordapp.com/attachments/1061798205278396416/1067854368365363350/image.png)

`cargo run -r scenes/emissive_perlin_spheres.json`

![emissive_perlin_spheres.json](https://cdn.discordapp.com/attachments/1061798205278396416/1067611982439653406/image.png)

