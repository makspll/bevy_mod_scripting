# updating to bevy 0.14 caused issues with WSL for me, these vars help, run source wsl.sh before other cargo commands and it might work, you might need to install mesa/vulkan drivers 
$env.WGPU_BACKEND = "vulkan" 
$env.WINIT_UNIX_BACKEND = "x11"