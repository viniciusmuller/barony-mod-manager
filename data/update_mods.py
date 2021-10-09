import os
import json
import asyncio

import aiohttp


STEAM_API_KEY = os.environ['STEAM_API_KEY']
BARONY_APP_ID = 371970
TARGET_FILE = "data/mods.json"


async def main():
    mods_data = []

    async with aiohttp.ClientSession() as session:
        async with session.get(
           'https://api.steampowered.com/IPublishedFileService/QueryFiles/v1',
           params={"appid": BARONY_APP_ID, "key": STEAM_API_KEY}
        ) as response:
            response_json = await response.json()
            total_mods = response_json["response"]["total"]

        tasks = [fetch_json(session, mods_data, i) for i in range(1, total_mods + 1)]
        await asyncio.gather(*tasks)
        write_data_to_file(TARGET_FILE, mods_data)


async def fetch_json(http_session, list_ref: list, mod_id: int) -> None:
        # Sleep a bit to avoid rate limit
        sleep_time = (mod_id * 30) / 1000
        await asyncio.sleep(sleep_time)

        async with http_session.get(
           'https://api.steampowered.com/IPublishedFileService/QueryFiles/v1',
            params={
                "appid": BARONY_APP_ID,
                "key": STEAM_API_KEY,
                "page": mod_id,
                "return_details": "true",
                "return_vote_data": "true",
                "strip_description_bbcode": "true",
            }
        ) as response:
            response_json = await response.json()
            mod = build_mod(response_json)
            list_ref.append(mod)


def build_mod(workshop_item_response: dict) -> dict:
    mod = workshop_item_response["response"]["publishedfiledetails"].pop()
    return {
        "id": mod["publishedfileid"],
        "title": mod["title"],
        "file_size": int(mod["file_size"]),
        "preview_url": mod["preview_url"],
        "description": mod["file_description"],
        "time_created": mod["time_created"],
        "time_updated": mod["time_updated"],
        "views": mod["views"],
        "favorited": mod["favorited"],
        "tags": [tag["tag"] for tag in mod["tags"]],
        "votes": {
            "up": mod["vote_data"]["votes_up"],
            "down": mod["vote_data"]["votes_down"],
        }
    }


def write_data_to_file(target: str, data: dict) -> None:
    with open(target, "w") as file:
        json.dump(data, file)


if __name__ == "__main__":
    asyncio.run(main())
