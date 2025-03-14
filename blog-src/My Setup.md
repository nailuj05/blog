# My Setup
I think my blog wouldn't be complete without me sharing my personal setup and work environment here as well.
e
## Hardware
For most of my work I publish here I use a [Framework 13 Laptop](https://frame.work/laptop13), it has served me well for over 2 years now. For those who don't know Framework, they build modular, fully repairable and upgradeable laptops. I am still using an 11th Gen Intel processor as I didn't feel the need for an update yet, though I am considering switching to the matte display for better readability in bright areas. 

## Operating System
I've been daily driving Fedora Linux on my laptop since the beginning, I used Ubuntu, DeepIn, Manjaro and Debian before, with all of them I had some issues, Fedora just worked for me so that's what I stuck with. I also use Windows 10 on my desktop, but I am working on switching to Linux there as well. Working and using Windows is just such a pain with Microsoft seemingly always trying to make the experience worse in any way they can - I am definitely not switching to Windows 11.

## Desktop
I started out using Gnome with [Pop Shell](https://github.com/pop-os/shell), which is a tiling window manager for it, on my laptop. After a while I didn't see why I need Gnome just to use it as a Tiling WM, so I switched to Hyprland. I don't have a real preference for Hyprland over i3 or sway.
Hyprland had all the customizability I was looking for, and I am happy with it. 
In general, I would recommend everyone to try out a Tiling Window Manager, especially on a laptop where dragging and arranging windows with a touchpad can be even more painful. 

## Editor
Like everyone else I started out with VSCode and a variety of IDEs for all the different languages I was using. I wasn't really happy with it, VSCode is slow, bloated, and I hate the plugin ecosystem. Searching for an alternative I switched to Neovim, I used Astrovim as a nice default config. Astrovim didn't make me feel content yet though, it wasn't my configuration and I felt like I wasn't in control. 

This is when I tried out Emacs, I know I am probably to use Emacs, but it was the first editor that really stuck with me. Even though it takes a while to configure it in a usable state it now does exactly what I want it to. I also still use vim bindings through evil-mode. 
Even though it isn't used that much today anymore, Emacs has a lot of nice features going for it, especially compilation-mode, org-mode, magit and dired are worth mentioning. 
I also use eglot which provides a lsp integration (though I configured it to be quite minimal). 

In general, I can only recommend to not stick with VSCode or whatever your current default is just because everyone uses it. You spend so much time in your editor when coding you should spend at least some time to find and configure a setup that works for you and isn't frustrating to use. While I can't recommend Emacs or Vim to everyone I can only urge you to try to find what works best for you.

You can find my dot files for most of my setup [here](https://github.com/nailuj05/dotfiles) if you want to take them as an inspiration, my Emacs theme can be found [here](https://github.com/nailuj05/gruber-darkest-theme).
