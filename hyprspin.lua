local PORTRAIT_MODES = { ["left-up"] = true, ["right-up"] = true }

function on_spin(ctx)
    if not ctx.monitor:find("eDP") then 
        return {} 
    end

    if ctx.orientation == "normal" then
        return {
            { action = "exec", args = "sh ~/.config/hypr/scripts/laptop-mode.sh" },
            { action = "keyword", args = "general:layout dwindle" }
        }
    end

    if PORTRAIT_MODES[ctx.orientation] then
        return {
            { action = "exec", args = "sh ~/.config/hypr/scripts/tablet-mode.sh" },
            { action = "keyword", args = "general:layout master" }
        }
    end

    if ctx.orientation == "bottom-up" then
        return {
            { action = "exec", args = "notify-send 'Orientation' 'Inverted (Tent Mode)'" }
        }
    end

    return {}
end
