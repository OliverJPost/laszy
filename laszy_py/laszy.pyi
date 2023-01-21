import numpy as np
from typing import Self

class PointCloud:
    @property
    def points(self) -> np.ndarray[np.float64]:
        """The point coordinates of the point cloud as a numpy array of shape (N, 3)"""
        ...

    @property
    def ground_points(self) -> np.ndarray[np.bool]:
        """A boolean array of shape (N,) indicating which points are ground points"""
        ...


class PointCloudBuilder:
    def from_file(self, filename: str) -> Self:
        """Configure the builder from a .las or .laz file."""
        ...

    def with_crop(self, lower_left: tuple[float, float], upper_right: tuple[float, float]) -> Self:
        """Configure the builder to crop the point cloud to the given rectangle."""
        ...


    def with_thinning_every_nth(self, nth: int) -> Self:
        """Configure the builder to thin the point cloud by keeping every nth point."""
        ...

    def with_thinning_random(self, keep_percentage: float) -> Self:
        """Configure the builder to thin the point cloud by randomly removing points."""
        ...


    def with_csf_ground_reclassification(
            self,
            rigidness: float,
            cloth_resolution: float,
            simulation_threshold: float,
            classification_threshold: float
    ) -> Self:
        """Configure the builder to reclassify ground points as CSF.

        Args:
            rigidness: The rigidness parameter for the CSF method. Must be between 0.0 and 1.0. 0.0 will classify all
                points as ground, 1.0 will classify points in a strict manner.
            cloth_resolution: The resolution of the cloth in meters.
            simulation_threshold: The threshold for the simulation to stop in meters.
            classification_threshold: The maximum distance between a point and the cloth for it to be classified as
                ground in meters.
        """
        ...

    def to_cloud(self) -> PointCloud:
        """Builds the point cloud using provided configuration and returns it."""
        ...

    def to_file(self, filename: str) -> None:
        """Write the point cloud to a file using the provided configuration.

        Args:
            filename: The filename to write to. Must be .las or .laz. When .laz is used, the file will be compressed.
        """
        ...

    def to_dtm_using_csf(self, filename: str, rigidness: float, cloth_resolution: float, distance_threshold: float) -> None:
        """Uses the CSF method to create a DTM from the point cloud. The DTM is written to the given filename.

        Args:
            filename: The filename to write the DTM to. Must end in .asc
            rigidness: The rigidness parameter for the CSF method. Must be between 0.0 and 1.0. 0.0 will classify all
                points as ground, 1.0 will classify points in a strict manner.
            cloth_resolution: The cloth resolution parameter for the CSF method in meters.
            distance_threshold: The distance threshold parameter for the CSF method that determines if the simulation
                should stop.
        """
        ...



