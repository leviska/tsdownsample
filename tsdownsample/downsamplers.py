# ------------------ Rust Downsamplers ------------------
import tsdownsample._rust.tsdownsample_rs as tsdownsample_rs

from .downsampling_interface import RustDownsamplingInterface

MinMaxDownsampler = RustDownsamplingInterface("MinMax", tsdownsample_rs.minmax)
M4Downsampler = RustDownsamplingInterface("M4", tsdownsample_rs.m4)
LTTBDownsampler = RustDownsamplingInterface("LTTB", tsdownsample_rs.lttb)
MinMaxLTTBDownsampler = RustDownsamplingInterface(
    "MinMaxLTTB", tsdownsample_rs.minmaxlttb
)

# ------------------ Function Downsamplers ------------------
import numpy as np

from .downsampling_interface import FuncDownsamplingInterface

MeanDownsampler = FuncDownsamplingInterface("Mean", np.mean)
MedianDownsampler = FuncDownsamplingInterface("Median", np.median)

# ------------------ EveryNth Downsampler ------------------
import math

import pandas as pd

from .downsampling_interface import DownsampleInterface


class _EveryNthDownsampler(DownsampleInterface):
    def __init__(self) -> None:
        super().__init__("EveryNth")

    def downsample(self, s: pd.Series, n_out: int, parallel: bool = False) -> pd.Series:
        return s[:: max(1, math.ceil(len(s) / n_out))]


EveryNthDownsampler = _EveryNthDownsampler()