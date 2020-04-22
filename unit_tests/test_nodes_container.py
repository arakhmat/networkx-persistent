import pyrpds
from networkx_persistent import NodesContainer

nodes = [0, "1", (3, "4")]


def get_test_nodes_container():
    nodes_container = NodesContainer()
    for node in nodes:
        nodes_container = nodes_container.add_node(node)
    return nodes_container


def test_constructor():
    nodes_container = NodesContainer()
    assert nodes_container == NodesContainer()


def test_add_node():
    nodes_container = get_test_nodes_container()

    assert len(nodes_container) == 3
    assert set(nodes_container) == {0, "1", (3, "4")}


def test_remove_node():
    nodes_container = get_test_nodes_container()

    assert len(nodes_container) == 3
    assert set(nodes_container) == {0, "1", (3, "4")}

    nodes_container = nodes_container.remove_node("1")
    assert len(nodes_container) == 2
    assert set(nodes_container) == {0, (3, "4")}

    nodes_container = nodes_container.remove_node(0)
    assert len(nodes_container) == 1
    assert set(nodes_container) == {(3, "4")}

    nodes_container = nodes_container.remove_node((3, "4"))
    assert len(nodes_container) == 0
    assert set(nodes_container) == set()


def test_contains():
    nodes_container = get_test_nodes_container()

    for node in nodes_container:
        assert node in nodes_container


def test_callable():
    nodes_container = get_test_nodes_container()

    for node in nodes_container():
        assert node in nodes_container


def test_get_item():
    nodes_container = get_test_nodes_container()

    for node in nodes_container:
        assert nodes_container[node] == pyrpds.pmap()
