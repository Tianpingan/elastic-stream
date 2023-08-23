from ducktape.tests.test import Test
from estest.services.pd import PD
from estest.services.range_server import RangeServer
from ducktape.utils.util import wait_until
class DemoTest(Test):
    def __init__(self, test_context):
        super(DemoTest, self).__init__(test_context=test_context)
        self.pd = PD(self.test_context, num_nodes=3)
        self.rs = RangeServer(test_context, num_nodes=3, pd=self.pd)
    def test_pd_and_range_server(self):
        self.pd.start()
        self.rs.start()

        self.pd.clean()
        self.rs.clean()
