import os
import shutil
import subprocess
from pathlib import Path

import typer
from rich.progress import Progress

app = typer.Typer()

# CHANGE THESE
BOOKDIR = r""
TEMPDIR = r""
BOOKEXT = ["cbz", "cbr", "cb7", "cbt"]
IMGEXT = ["jpg", "jpeg", "png", "gif"]

@app.command()
def dirs():  # noqa: C901
    """
    Encode images into jpegxl
    """

    Path(TEMPDIR).mkdir(parents=True, exist_ok=True)
    os.chdir(TEMPDIR)

    # Initialize progress bar
    with Progress() as progress:
        # Iterate over file extensions
        for bext in BOOKEXT:
            # count = 0
            first_directory = True
            first_file = True
            first_image = True

            # Iterate over folders and files
            for root, dirs, files in os.walk(BOOKDIR):
                if first_directory:
                    # Display the progress bar for the directories traversed
                    task1 = progress.add_task(
                        f"[red]Converting {bext} folder progress...", total=len(dirs)
                    )
                    first_directory = False

                # Update the directory progress
                if not progress.finished:
                    progress.update(task1, advance=1)

                for file in files:
                    if first_file:
                        task2 = progress.add_task(
                            f"[red]Converting {bext} files in folder {file[:-8]}...",
                            total=len(files),
                        )
                        first_file = False

                    # Update the folder progress
                    if not progress.finished:
                        progress.update(task2, advance=1)
                        # time.sleep(0.05)

                    if file.lower().endswith("." + bext):
                        book_path = Path(root) / file

                        shutil.move(
                            book_path,
                            Path(TEMPDIR) / Path(book_path).name,
                        )

                        subprocess.run(
                            ["7z", "x", Path(book_path).name],  # noqa: S603, S607
                            check=True,
                            stdout=subprocess.DEVNULL,
                        )

                        Path(Path(book_path).name).unlink()
                        shutil.rmtree("./__MACOSX/", ignore_errors=True)

                        # subprocess.run(["jpegoptim", "-s", "*.jpg"], stderr=subprocess.DEVNULL, shell=True)
                        # subprocess.run(["jpegoptim", "-s", "*.jpeg"], stderr=subprocess.DEVNULL, shell=True)

                        for iext in IMGEXT:
                            jxl_command = "cjxl -j 1"

                            # TODO: convert get-png-strategy into an python module
                            # if iext == "png":
                            #     jxl_command = subprocess.run(
                            #         [
                            #             "node",
                            #             r".\get-png-strategy.js",
                            #         ],
                            #         capture_output=True,
                            #         text=True,
                            #         shell=True,
                            #     ).stdout.strip()

                            for new_root, _new_dirs, new_files in os.walk("."):
                                first_image = True

                                for new_file in new_files:
                                    if first_image:
                                        task3 = progress.add_task(
                                            f"[red]Converting {iext} images in {file} using command {jxl_command}...",
                                            total=len(new_files),
                                        )
                                        first_image = False

                                    # Update the progress for images converted
                                    if not progress.finished:
                                        progress.update(task3, advance=1)

                                    if new_file.lower().endswith("." + iext):
                                        file_path = Path(new_root) / new_file

                                        #TODO: Remove usage of os.path.splittext
                                        # fmt: off
                                        os.system(
                                            f"{jxl_command} --quiet \"{file_path}\" \"{os.path.splitext(file_path)[0]}.jxl\" && del /F /Q \"{file_path}\""  # noqa: S605, PTH122
                                        )
                                        # fmt: on

                                progress.remove_task(task3)

                        #TODO: Remove usage of os.path.splittext
                        subprocess.run(
                            [  # noqa: S607
                                "7z",
                                "a",
                                "-tzip",
                                f"{os.path.splitext(book_path)[0]}.cbz",  # noqa: PTH122
                                "*",
                            ],
                            shell=True,  # noqa: S602
                            check=True,
                            stdout=subprocess.DEVNULL,
                        )

                        shutil.rmtree(TEMPDIR, ignore_errors=True)

                try:
                    progress.remove_task(task2)
                except:  # noqa: E722, S112
                    continue

                first_file = True


if __name__ == "__main__":
    app()
