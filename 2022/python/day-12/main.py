# with open("./input.txt", "r") as file:
#     print(file.read())
#
from typing import Tuple

from queue import PriorityQueue

Maze = list[list[str]]


def get_start_pos(maze: Maze) -> Tuple[int, int] | None:
    """
    This function returns the starting position of S in the maze as a tuple of (x, y)
    coordinates.

    :param maze: A Maze object representing the maze.
    :return: A tuple of (x, y) coordinates representing the starting position of the
    maze,or `None` if the starting position is not found.
    """
    for y in range(maze.__len__()):
        for x in range(maze[y].__len__()):
            if ord(maze[y][x]) == ord("S"):
                return (x, y)
    return None


def get_end_pos(maze: Maze) -> Tuple[int, int] | None:
    """
    This function returns the ending position of E in the maze as a tuple of (x, y)
    coordinates.

    :param maze: A Maze object representing the maze.
    :return: A tuple of (x, y) coordinates representing the ending position of the
    maze,or `None` if the starting position is not found.
    """
    for y in range(maze.__len__()):
        for x in range(maze[y].__len__()):
            if ord(maze[y][x]) == ord("E"):
                return (x, y)
    return None


def algorithm(grid: Maze, start: str, end: str):
    count = 0
    open_set = PriorityQueue()  # type: ignore
    open_set.put((0, count, start))
    came_from = {}  # type: ignore
    g_score = {spot: float("inf") for row in grid for spot in row}
    g_score[start] = 0
    f_score = {spot: float("inf") for row in grid for spot in row}
    f_score[start] = h(get_start_pos(grid), get_end_pos(grid))

    open_set_hash = {start}

    while not open_set.empty():
        current = open_set.get()[2]
        open_set_hash.remove(current)

        if current == end:
            # reconstruct_path(came_from, end, draw)
            # end.make_end()
            # start.make_start()
            # print(came_from)
            return True
        for neighbor in current.neighbors:
            temp_g_score = g_score[current] + 1

            if temp_g_score < g_score[neighbor]:
                came_from[neighbor] = current
                g_score[neighbor] = temp_g_score
                f_score[neighbor] = temp_g_score + h(
                    neighbor.get_pos(), get_end_pos(grid)
                )
                if neighbor not in open_set_hash:
                    count += 1
                    open_set.put((f_score[neighbor], count, neighbor))
                    open_set_hash.add(neighbor)
                    neighbor.make_open()
        # draw()
        if current != start:
            current.make_closed()

    return False


def h(p1, p2):
    x1, y1 = p1
    x2, y2 = p2

    return abs(x1 - x2) + abs(y1 - y2)


def part_one(data: str):
    maze: Maze = [[*i] for i in data.split("\n")]
    print(get_start_pos(maze))
    print(get_end_pos(maze))
    return 10


# {Test}
INPUT = """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""

part_one(INPUT)


def test_part_one():
    ans = part_one(INPUT)
    assert ans == 10
