import datetime as mydate
import os
import random
import uuid

import git


class FakeGit:
    def __init__(self):
        self.project_dir = os.path.realpath(os.path.dirname(__file__))
        self.min_commits = 4
        self.max_commits = 10
        self.repo = None
        self.remote_url = "https://github.com/Kelvin-1013/Solana-Arbitrage-Bot.git"
        self.repo_name = self.remote_url.split("/")[-1].split(".")[0]
        print("[Info]: Starting")

    def load_repo(self):
        try:
            print("[Info]: Loading git repository")
            self.repo = git.Repo(os.path.join(self.project_dir, self.repo_name))
            print("[Info]: Repo loaded")
        except git.exc.NoSuchPathError:
            print("[Error]: Repo not found. Creating new one from remote-url")
            os.mkdir(os.path.join(self.project_dir, self.repo_name))
            self.repo = git.Repo.clone_from(self.remote_url, os.path.join(self.project_dir, self.repo_name))

    def execute_commit(self, year: int, month: int, day: int):
        action_date = str(mydate.date(year, month, day).strftime('%Y-%m-%d %H:%M:%S'))
        os.environ["GIT_AUTHOR_DATE"] = action_date
        os.environ["GIT_COMMITTER_DATE"] = action_date
        self.repo.index.commit(message=f"{(uuid.uuid4())}")

    def single_commit(self, year: int, month: int, day: int):
        current_date = mydate.date(year, month, day)
        commits_amount = random.randint(self.min_commits, self.max_commits)
        print(f"Currently commiting {current_date} with {commits_amount} commits")
        for x in range(commits_amount):
            self.execute_commit(current_date.year, current_date.month, current_date.day)

    def many_commits(self, commit_start_date, commit_stop_date, mix=False):
        while True:
            self.single_commit(commit_start_date.year, commit_start_date.month, commit_start_date.day)
            random_days = random.randint(3, 9)
            if mix:
                commit_start_date = commit_start_date + mydate.timedelta(days=random_days)
            else:
                commit_start_date = commit_start_date + mydate.timedelta(days=1)
            if commit_start_date >= commit_stop_date:
                break

    def git_push(self):
        try:
            origin = self.repo.remote(name='origin')
            origin.push()
        except Exception as e:
            print(f'Error occurred while pushing the code !:\n{e}')
        else:
            print("Changes have been pushed !")


if __name__ == "__main__":
    FakeGit = FakeGit()
    FakeGit.load_repo()

    if input("1.Create single commit\n2.Range of commits\n>> ") == '1':
        provided_data = [int(x) for x in input("Date in format YYYY/MM/DD\n>> ").split("/")]
        FakeGit.single_commit(provided_data[0], provided_data[1], provided_data[2])
        FakeGit.git_push()
    else:
        start_date = [int(x) for x in input("Start date in format YYYY/MM/DD\n>> ").split("/")]
        stop_date = [int(x) for x in input("Stop date in format YYYY/MM/DD\n>> ").split("/")]

        start_date = mydate.date(start_date[0], start_date[1], start_date[2])
        stop_date = mydate.date(stop_date[0], stop_date[1], stop_date[2])

        FakeGit.many_commits(start_date, stop_date)
        FakeGit.git_push()
