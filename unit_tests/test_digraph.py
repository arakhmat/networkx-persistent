import pyrpds
from networkx_persistent import DiGraph

nodes = [0, "1", (3, "4")]


def get_test_graph():
    graph = DiGraph()
    for node in nodes:
        graph = graph.add_node(node)

    for node in nodes:
        for other_node in nodes:
            if node != other_node:
                graph = graph.add_edge(node, other_node)
    return graph


def test_constructor():
    graph = DiGraph()
    assert graph == DiGraph()


def test_add_node():
    graph = get_test_graph()

    assert len(graph) == 3
    assert set(graph) == {0, "1", (3, "4")}


def test_remove_node():
    graph = get_test_graph()

    assert len(graph) == 3
    assert set(graph) == {0, "1", (3, "4")}

    graph = graph.remove_node("1")
    assert len(graph) == 2
    assert set(graph) == {0, (3, "4")}

    graph = graph.remove_node(0)
    assert len(graph) == 1
    assert set(graph) == {(3, "4")}

    graph = graph.remove_node((3, "4"))
    assert len(graph) == 0
    assert set(graph) == set()


def test_contains():
    graph = get_test_graph()

    for node in graph:
        assert node in graph


def test_get_item():
    graph = get_test_graph()

    for node in graph:
        assert graph[node] == pyrpds.pmap()


def test_add_edge():
    graph = get_test_graph()
    edges = list(graph.edges())
    assert len(edges) == 3
    for neighbors in edges:
        assert len(neighbors) == 2


def test_remove_edge():
    graph = get_test_graph()
    edges = list(graph.edges())
    assert len(edges) == 3

    graph_with_removed_edge = graph.remove_edge(0, "1")
    edges = list(graph_with_removed_edge.edges())
    assert len(edges) == 3
