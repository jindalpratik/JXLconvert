import typer
import os
import subprocess
import shutil
from rich.progress import Progress

app = typer.Typer()

# CHANGE THESE
BOOKDIR = r""
TEMPDIR = r""
BOOKEXT = ["cbz", "cbr", "cb7", "cbt"]
IMGEXT = ["jpg", "jpeg", "png", "gif"]


@app.command()
def dirs():
    """
    Command to encode images into jpegxl
    """

    os.makedirs(TEMPDIR, exist_ok=True)
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

                else:
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
                            book_path = os.path.join(root, file)

                            shutil.move(
                                book_path,
                                os.path.join(TEMPDIR, os.path.basename(book_path)),
                            )

                            subprocess.run(
                                ["7z", "x", os.path.basename(book_path)],
                                shell=True,
                                check=True,
                                stdout=subprocess.DEVNULL,
                            )

                            os.remove(os.path.basename(book_path))
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

                                for new_root, new_dirs, new_files in os.walk("."):
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
                                            file_path = os.path.join(new_root, new_file)

                                            # fmt: off
                                            os.system(
                                                f"{jxl_command} --quiet \"{file_path}\" \"{os.path.splitext(file_path)[0]}.jxl\" && del /F /Q \"{file_path}\""
                                            )
                                            # fmt: on

                                    progress.remove_task(task3)

                            subprocess.run(
                                [
                                    "7z",
                                    "a",
                                    "-tzip",
                                    f"{os.path.splitext(book_path)[0]}.cbz",
                                    "*",
                                ],
                                shell=True,
                                check=True,
                                stdout=subprocess.DEVNULL,
                            )

                            shutil.rmtree(TEMPDIR, ignore_errors=True)

                    progress.remove_task(task2)
                    first_file = True


if __name__ == "__main__":
    app()
