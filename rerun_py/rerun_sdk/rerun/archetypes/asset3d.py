# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/asset3d.fbs".

# You can extend this class by creating a "Asset3DExt" class in "asset3d_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import Archetype
from .asset3d_ext import Asset3DExt

__all__ = ["Asset3D"]


@define(str=False, repr=False, init=False)
class Asset3D(Asset3DExt, Archetype):
    """
    A prepacked 3D asset (`.gltf`, `.glb`, `.obj`, etc.).

    Examples
    --------
    Simple 3D asset:
    ```python
    import sys

    import rerun as rr

    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <path_to_asset.[gltf|glb]>")
        sys.exit(1)

    rr.init("rerun_example_asset3d_simple", spawn=True)

    rr.log("world", rr.ViewCoordinates.RIGHT_HAND_Z_UP, timeless=True)  # Set an up-axis
    rr.log("world/asset", rr.Asset3D(sys.argv[1]))
    ```

    3D asset with out-of-tree transform:
    ```python
    import sys

    import numpy as np
    import rerun as rr
    from rerun.components import OutOfTreeTransform3DBatch
    from rerun.datatypes import TranslationRotationScale3D

    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <path_to_asset.[gltf|glb]>")
        sys.exit(1)

    rr.init("rerun_example_asset3d_out_of_tree", spawn=True)

    rr.log("world", rr.ViewCoordinates.RIGHT_HAND_Z_UP, timeless=True)  # Set an up-axis

    rr.set_time_sequence("frame", 0)
    rr.log("world/asset", rr.Asset3D(sys.argv[1]))
    # Those points will not be affected by their parent's out-of-tree transform!
    rr.log(
        "world/asset/points",
        rr.Points3D(np.vstack([xyz.ravel() for xyz in np.mgrid[3 * [slice(-10, 10, 10j)]]]).T),
    )

    asset = rr.Asset3D(sys.argv[1])
    for i in range(1, 20):
        rr.set_time_sequence("frame", i)

        translation = TranslationRotationScale3D(translation=[0, 0, i - 10.0])
        rr.log_components("world/asset", [OutOfTreeTransform3DBatch(translation)])
    ```
    """

    # __init__ can be found in asset3d_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            blob=None,  # type: ignore[arg-type]
            media_type=None,  # type: ignore[arg-type]
            transform=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> Asset3D:
        """Produce an empty Asset3D, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    blob: components.BlobBatch = field(
        metadata={"component": "required"},
        converter=components.BlobBatch._required,  # type: ignore[misc]
    )
    """
    The asset's bytes.
    """

    media_type: components.MediaTypeBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.MediaTypeBatch._optional,  # type: ignore[misc]
    )
    """
    The Media Type of the asset.

    For instance:
    * `model/gltf-binary`
    * `model/obj`

    If omitted, the viewer will try to guess from the data blob.
    If it cannot guess, it won't be able to render the asset.
    """

    transform: components.OutOfTreeTransform3DBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.OutOfTreeTransform3DBatch._optional,  # type: ignore[misc]
    )
    """
    An out-of-tree transform.

    Applies a transformation to the asset itself without impacting its children.
    """

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__


if hasattr(Asset3DExt, "deferred_patch_class"):
    Asset3DExt.deferred_patch_class(Asset3D)
