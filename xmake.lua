add_rules("mode.debug", "mode.release")

set_languages("c23", "c++23")
add_rules("plugin.compile_commands.autoupdate", { outputdir = "." })

target("pomodoro", function()
	set_kind("binary")
	add_files("src/*.cpp")
	add_includedirs("include")
end)
