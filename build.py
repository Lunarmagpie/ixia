from mypyc.build import mypycify


ext_modules = mypycify(["src/ixia/lib.py"])


def build(setup_kwargs):
    setup_kwargs.update({"ext_modules": ext_modules})
