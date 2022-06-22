local record global_types
--This is module level documentation for our api, it will be shown first
--
--
--
	record APIModule
		userdata

		-- Pure functions
		--Here we document the next function
		--
		--## Markdown!:
		--
		--```lua
		--                local hello = "string"  
		--            
		--```
		my_function: function():(string)

		help: function(string):(string)


	end
end
global my_api: global_types.APIModule
return global_types