from sarus_data_spec.constants import BIG_DATA_TASK, IS_BIG_DATA
from sarus_data_spec.context.worker import WorkerContext
from sarus_data_spec.dataset import sql
from sarus_data_spec.transform import (
    assign_budget,
    attributes_budget,
    automatic_budget,
    automatic_protected_paths,
    automatic_public_paths,
    automatic_user_settings,
    protect,
    user_settings,
)
from sarus_data_spec.protobuf.utilities import json
import sarus_data_spec.status as stt

DIRNAME = "./tests/extract/"

with WorkerContext() as context:
    URI = 'postgresql://postgres:pyqrlew-db@localhost:5433/postgres'
    ds = sql(uri=URI, tables=[('extract', 'beacon'), ('extract', 'census')])

    # all external statuses have to be set for all managers sharing
    # the storage
    stt.ready(ds, task=BIG_DATA_TASK, properties={IS_BIG_DATA: str(False)})
    paths = automatic_protected_paths()(ds)
    public_entities = automatic_public_paths()(ds)
    protected_ds = protect()(
        ds, protected_paths=paths, public_paths=public_entities
    )
    user_type = automatic_user_settings()(protected_ds)
    final_ds = user_settings()(protected_ds, user_type=user_type)

    # BUDGETS Definitions
    total_budget = automatic_budget()(final_ds)
    attr_budget = attributes_budget()(total_budget)
    ds_with_budget = assign_budget()(final_ds, attributes_budget=attr_budget)

    #Write file
    with open(DIRNAME + 'dataset.txt', 'w') as file:
        file.write(json(ds_with_budget.protobuf()))
    with open(DIRNAME + 'schema.txt', 'w') as file:
        file.write(json(ds_with_budget.schema().protobuf()))
    with open(DIRNAME + 'size.txt', 'w') as file:
        file.write(json(ds_with_budget.size().protobuf()))
