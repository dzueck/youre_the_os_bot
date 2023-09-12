This is a bot for the game You're the OS
https://drfreckles42.itch.io/youre-the-os

This bot should work on any windows computer as long as the game is fullscreen on your primary monitor.
The bot has 3 run modes. run, verify, and print.

Run mode is the standard way of running the bot. It will find your primary monitor (where the game should be running) and it will play the game there.
The first arg is the number of cpus and the second arg is the number of ram rows.
cargo run --release -- run 16 4 (example insane run command)

Verify mode is used to ensure that the bot will work on your computer. It takes a screenshot of the game and places blue crosses where it belives the important points are.
To use it, run the game and play for a few seconds. You want to populate your ram and some of the disk. The more you populate, the more you can verify it works.
You also want to get some processes that are waiting for IO as the points of interest for IO should be the top of their hourglass. Once all of this is filled in, run the command.
The first arg is the number of cpus, the second arg is the number of ram rows, and the third command is the path for the output image.
cargo run --release -- verify 16 4 ./verification_image.png (example insane run command)
Once this is run, view the image and make sure the blue crosses are in the correct place. The crosses for the processes should be on the tops of the hourglasses specifically in the grey part.
The Page crosses should be in the box anywhere other than the text. There should also be one on the I/O Events button but this one may be hard to see and is likely fine for anyone.
For an example correct image see correct_poses.png.
The reason you might want to verify this is correct is that I use percentages of the screen that get converted to pixles. This allows it to run on any machine, but it was only tested on mine.
If the percentages are slightly off it could be a problem on other machines due to rounding. If the crosses are not in the right spot on your machine you can modify the values in game_manager.rs.
You can modify the start values or the increment values slightly until it works on your machine. Verify it works with the verify command. To get the correct values, you may want to use the print command.

Print mode is used to get pixel positions on the screen. While running it will contantly print out your cursors x, y, x%, and y% values. The code only uses x% and y% values for the most part.
This command is used to get the values to modify the pixel values in game_manager.rs. Note, these values may not be right if your main display does not contain the origin position.
