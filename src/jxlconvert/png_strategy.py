import random
import subprocess
import time
from pathlib import Path


class PngStrategy:
    def __init__(self, jxl_folder, cjxl_lossy, cjxl_lossless):
        self.sample_size = 3
        self.min_png_files = 15

        # Input parameters
        self.jxl_folder = jxl_folder
        self.cjxl_lossy = cjxl_lossy
        self.cjxl_lossless = cjxl_lossless

    def __get_png_files(self):
        return list(Path(self.jxl_folder).rglob("*.png"))

    def __get_random_number(self, start, end):
        return random.randint(  # noqa: S311 Disable requirement of cryptographic random int generator
            start, end
        )

    # Function to run a jxl command using subprocess
    # TODO: see of this can be moved into an utility module as is also used in __main__.py
    def __run_jxl_command(self, jxl_command, original_path, converted_path):
        args = jxl_command.split(" ")
        args.append("--quiet")
        args.append(original_path)
        args.append(converted_path)
        # Route the stdout and stderr to null so as not to get a output on the terminal
        subprocess.Popen(
            args=args, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
        )

    def png_strategy(self):  # noqa: C901

        try:
            # Get the png files in the tmp directory and store them in a list
            png_files = self.__get_png_files()

            # Return default values if png files less than minimum required
            if len(png_files) == 0:
                return self.cjxl_lossy

            if len(png_files) <= self.min_png_files:
                return self.cjxl_lossless

            random_png_files = []

            # Get pages at random to test
            while len(random_png_files) < self.sample_size:
                random_number = self.__get_random_number(6, len(png_files) - 5)
                if (random_number - 1) not in random_png_files:
                    random_png_files.append(random_number - 1)

            converted_files = []
            for i in random_png_files:
                base_path = png_files[i]
                lossy_path = Path(str(base_path).replace(".png", "-lossy.jxl").strip('\n'))
                lossless_path = Path(str(base_path).replace(".png", "-lossless.jxl").strip('\n'))

                self.__run_jxl_command(self.cjxl_lossy, base_path, lossy_path)
                self.__run_jxl_command(self.cjxl_lossless, base_path, lossless_path)

                # Wait for the files to be fully written to disk
                while not lossy_path.exists() or not lossless_path.exists():
                    time.sleep(0.1)

                converted_files.append(
                    {
                        'base': {'size': Path(base_path).stat().st_size, 'path': base_path},
                        'lossy': {'size': Path(lossy_path).stat().st_size, 'path': lossy_path},
                        'lossless': {'size': Path(lossless_path).stat().st_size, 'path': lossless_path},
                    }
                )
        except Exception as _e:
            # Use lossless if there is an error
            return self.cjxl_lossless
        else:
            # Use lossy
            # TODO: Move this into a seperate function
            if all(file['lossy']['size'] < file['lossless']['size'] for file in converted_files):
                for file in converted_files:
                    Path.unlink(file['base']['path'])
                    Path.unlink(file['lossless']['path'])
                return self.cjxl_lossy

            for file in converted_files:
                Path.unlink(file['base']['path'])
                Path.unlink(file['lossy']['path'])
            return self.cjxl_lossless
