# gm_cron
Cron Jobs in Garry's Mod.

## Example
```lua
require("cron")

local job
local count = 0
job = CronJob("1/10 * * * * *", function()
	print("I get executed every 10s!")

    	count = count + 1
	if count > 3 then
		print("Removed after 30s!")
		job:Remove()
	end
end)

PrintTable(job)
```

## Compiling
- Open a terminal
- Install **cargo** if you dont have it (on Windows => https://win.rustup.rs) (on Linux/Macos => curl https://sh.rustup.rs -sSf | sh)
- Get [git](https://git-scm.com/downloads) or download the archive for the repository directly
- `git clone https://github.com/Earu/gm_cron` (ignore this if you've downloaded the archive)
- Run `cd gm_cron`
- `cargo build`
- Go in `target/debug` and rename the binary according to your branch and realm (gmsv_cron_win64.dll, gmcl_cron_win64.dll, gmsv_cron_linux.dll, gmcl_cron_linux.dll, gmcl_cron_osx64.dll)
- Put the binary in your gmod `lua/bin` directory

*Note: Even on other platforms than Windows the extension of your modules **needs** to be **.dll***
