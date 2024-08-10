function read_lines(file)
    local lines = {}
    for line in io.lines(file) do
        lines[#lines + 1] = line
    end

    return lines
end

local file_name = arg[1]
local needle = arg[2]

local lines = read_lines(file_name)

for _, l in pairs(lines) do
    if string.match(l, needle) then
        print(l)
    end
end
