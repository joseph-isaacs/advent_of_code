const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const x = try std.fs.cwd().realpathAlloc(alloc, ".");
    std.debug.print("{s}\n", .{x});
    var file = try std.fs.cwd().openFile("inputs/p1/p.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;
    var l = [_]u8{0} ** 1024;
    var pos: u8 = 0;
    var sum: u32 = 0;

    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        for (line) |char| {
            if (std.ascii.isDigit(char)) {
                l[pos] = char;
                pos += 1;
            }
        }
        l[1] = l[pos-1];
        std.debug.print("{s}\n", .{l[0..2]});
        sum += try std.fmt.parseInt(u32, l[0..2], 10);
        pos = 0;
    }
    std.debug.print("sum: {d}\n", .{sum});
}