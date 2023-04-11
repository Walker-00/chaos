<div align="center">

# Chaos (Cha-Os)

#

<div>
<img alt="GitHub" src="https://img.shields.io/github/license/Walker-00/chaos?color=red&style=flat-square">
<img alt="GitHub language count" src="https://img.shields.io/github/languages/count/Walker-00/chaos?color=red&logo=rust&logoColor=red&style=flat-square">
<img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/Walker-00/chaos?color=red&logo=github&style=flat-square">
</div>

#

<h3>

Just a fun project to build an Operating System from fully scratch with rust

Bootloader is written is assembly and
Kernel is written in rust

</h3>

#

## [Bootloader Features]
<h3>

- Multiboot
- Real Mode (16 bits)
- Pagging<br>
- Memory Mapping
- Global Descriptor Table loading
- Protected Mode(x86 32bits)
- Long Mode (x86_64 bits)
- Copy Kernel from disk to Protected Memory
- Stack Mem
</h3>

#

## [Kernel Features]

<h3>

- VGA Driver
- print! and println! macros to write text to VGF text buffer

</h3>

#

## [More To Come]

<h3>

- Keyboard Driver
- CPU exceptions handler
- Programmable Interrupt Controller Driver
- Keyboard Driver 

</h3>

#

## [Building]

```
git clone https://github.com/Walker-00/chaos
cd chaos
make build
make run
```

#

## [Note]

<h4>

If you got some issues like header missing or multiboot header missing!<br>
Please run make clean and make build in different shell.<br>
Or If you got any other issues please report me.

<h4>

#

![Jokes Card](https://readme-jokes.vercel.app/api?theme=tokyonight)

#
## [Contributors]

<a href="https://github.com/Walker-00/chaos/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Walker-00/chaos" />
</a>


</div>
